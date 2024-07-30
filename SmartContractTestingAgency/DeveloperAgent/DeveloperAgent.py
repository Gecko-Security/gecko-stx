from agency_swarm.agents import Agent
from agency_swarm.tools import CodeInterpreter

class DeveloperAgent(Agent):
    def __init__(self):
        super().__init__(
            name="DeveloperAgent",
            description="Implements the invariants in TypeScript using fast-check to test the Clarity smart contracts.",
            instructions="./instructions.md",
            files_folder="./files",
            schemas_folder="./schemas",
            tools=[CodeInterpreter],
            tools_folder="./tools",
            temperature=0.3,
            max_prompt_tokens=25000,
        )
        
    def response_validator(self, message):
        return message
