import logging
from hypothesis import given, settings, strategies as st, event, Verbosity
from hypothesis.stateful import RuleBasedStateMachine, rule, invariant
from dataclasses import dataclass

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(message)s')
logger = logging.getLogger(__name__)

@dataclass
class ClarityContract:
    counter: int = 0

    def get_counter(self):
        return self.counter

    def increment(self):
        self.counter += 1
        return {"okay": True}

    def decrement(self):
        if self.counter > 0:
            self.counter -= 1
            return {"okay": True}
        return {"error": "ERR_COUNTER_MUST_BE_POSITIVE"}

    def add(self, n):
        if n > 1:
            self.counter += n
            return {"okay": True}
        return {"error": "ERROR_ADD_MORE_THAN_ONE"}

class CounterStateMachine(RuleBasedStateMachine):
    def __init__(self):
        super().__init__()
        self.contract = ClarityContract()
        self.model_counter = 0
        self.steps = 0

    @rule()
    def increment(self):
        self.steps += 1
        logger.info(f"Step {self.steps}: Incrementing counter")
        result = self.contract.increment()
        assert result == {"okay": True}
        self.model_counter += 1
        event("increment")

    @rule()
    def decrement(self):
        self.steps += 1
        logger.info(f"Step {self.steps}: Decrementing counter")
        result = self.contract.decrement()
        if self.model_counter > 0:
            assert result == {"okay": True}
            self.model_counter -= 1
            event("decrement_success")
        else:
            assert result == {"error": "ERR_COUNTER_MUST_BE_POSITIVE"}
            event("decrement_error")

    @rule(n=st.integers(min_value=2, max_value=1000))
    def add(self, n):
        self.steps += 1
        logger.info(f"Step {self.steps}: Adding {n} to counter")
        result = self.contract.add(n)
        assert result == {"okay": True}
        self.model_counter += n
        event(f"add_{n}")

    @rule(n=st.integers(max_value=1))
    def add_error(self, n):
        self.steps += 1
        logger.info(f"Step {self.steps}: Attempting to add {n} (should fail)")
        result = self.contract.add(n)
        assert result == {"error": "ERROR_ADD_MORE_THAN_ONE"}
        event("add_error")

    @invariant()
    def counter_matches(self):
        contract_counter = self.contract.get_counter()
        assert contract_counter == self.model_counter, \
            f"Contract counter {contract_counter} doesn't match model counter {self.model_counter}"
        logger.info(f"Invariant check: Contract counter = {contract_counter}, Model counter = {self.model_counter}")

@settings(max_examples=1000, stateful_step_count=50, deadline=None, verbosity=Verbosity.verbose)
@given(st.data())
def test_counter_state_machine(data):
    CounterStateMachine().execute_step(data.draw)

if __name__ == "__main__":
    logger.info("Starting Counter State Machine Test")
    test_counter_state_machine()
    logger.info("Counter State Machine Test Completed")