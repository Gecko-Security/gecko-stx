from agency_swarm.agents import Agent


class PlannerAgent(Agent):
    def __init__(self):
        super().__init__(
            name="PlannerAgent",
            description="Plans which invariants need to be developed for property-based testing.",
            instructions="./instructions.md",
            files_folder="./files",
            schemas_folder="./schemas",
            tools=[],
            tools_folder="./tools",
            temperature=0.3,
            max_prompt_tokens=25000,
        )
        
    def response_validator(self, message):
        return message
