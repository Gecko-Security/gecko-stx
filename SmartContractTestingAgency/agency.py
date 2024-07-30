from agency_swarm import Agency
from DeveloperAgent import DeveloperAgent
from BrowsingAgent import BrowsingAgent
from DeveloperAgent import DeveloperAgent
from PlannerAgent import PlannerAgent
from SmartContractCEO import SmartContractCEO


agency = Agency([ceo, planner, developer, [ceo, planner],
 [ceo, developer],
 [planner, developer],
 [planner, browser],
 [developer, browser]],
                shared_instructions='./agency_manifesto.md', # shared instructions for all agents
                max_prompt_tokens=25000, # default tokens in conversation for all agents
                temperature=0.3, # default temperature for all agents
                )
                
if __name__ == '__main__':
    agency.demo_gradio()
