from agency_swarm.tools import BaseTool
from pydantic import Field
import subprocess
import os
import json

class ClaritySdkTool(BaseTool):
    """
    This tool provides the capability to interact with Clarity smart contracts.
    It supports deploying, calling, and querying Clarity smart contracts on the Stacks blockchain.
    """

    action: str = Field(
        ..., description="The action to perform: 'deploy', 'call', or 'query'."
    )
    contract_name: str = Field(
        ..., description="The name of the Clarity smart contract."
    )
    contract_code: str = Field(
        None, description="The Clarity smart contract code (required for 'deploy' action)."
    )
    function_name: str = Field(
        None, description="The function name to call or query (required for 'call' and 'query' actions)."
    )
    function_args: str = Field(
        None, description="The arguments for the function call or query in JSON format (required for 'call' and 'query' actions)."
    )
    sender_address: str = Field(
        None, description="The sender address for deploying or calling the contract (required for 'deploy' and 'call' actions)."
    )

    def run(self):
        """
        The implementation of the run method, where the tool's main functionality is executed.
        This method deploys, calls, or queries the provided Clarity smart contract.
        """
        try:
            if self.action not in ['deploy', 'call', 'query']:
                return "Invalid action. Please specify 'deploy', 'call', or 'query'."

            if self.action == 'deploy':
                if not self.contract_code or not self.sender_address:
                    return "For 'deploy' action, 'contract_code' and 'sender_address' are required."

                # Deploy the Clarity smart contract
                deploy_result = subprocess.run(
                    ["clarity-cli", "launch", self.contract_name, self.contract_code, self.sender_address],
                    capture_output=True,
                    text=True
                )

                if deploy_result.returncode != 0:
                    return f"Deployment error:\n{deploy_result.stderr}"

                return f"Contract deployed successfully:\n{deploy_result.stdout}"

            elif self.action in ['call', 'query']:
                if not self.function_name or not self.function_args:
                    return f"For '{self.action}' action, 'function_name' and 'function_args' are required."

                function_args_list = json.loads(self.function_args)

                if self.action == 'call':
                    if not self.sender_address:
                        return "For 'call' action, 'sender_address' is required."

                    # Call the Clarity smart contract function
                    call_result = subprocess.run(
                        ["clarity-cli", "execute", self.contract_name, self.function_name, self.sender_address] + function_args_list,
                        capture_output=True,
                        text=True
                    )

                    if call_result.returncode != 0:
                        return f"Function call error:\n{call_result.stderr}"

                    return f"Function called successfully:\n{call_result.stdout}"

                elif self.action == 'query':
                    # Query the Clarity smart contract function
                    query_result = subprocess.run(
                        ["clarity-cli", "query", self.contract_name, self.function_name] + function_args_list,
                        capture_output=True,
                        text=True
                    )

                    if query_result.returncode != 0:
                        return f"Function query error:\n{query_result.stderr}"

                    return f"Function queried successfully:\n{query_result.stdout}"

        except Exception as e:
            return f"An error occurred: {str(e)}"