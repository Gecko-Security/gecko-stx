from agency_swarm.agents import Agent


class SmartContractCEO(Agent):
    def __init__(self):
        super().__init__(
            name="SmartContractCEO",
            description="Oversees the entire process and coordinates between agents.",
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
