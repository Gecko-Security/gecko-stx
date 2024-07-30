from agency_swarm.tools import BaseTool
from pydantic import Field
import subprocess
import os
import tempfile

class TypeScriptTool(BaseTool):
    """
    This tool provides the capability to write, compile, and run TypeScript code.
    It supports the latest TypeScript features and provides error messages for any issues in the code.
    """

    typescript_code: str = Field(
        ..., description="The TypeScript code to be compiled and executed."
    )

    def run(self):
        """
        The implementation of the run method, where the tool's main functionality is executed.
        This method compiles and runs the provided TypeScript code.
        """
        try:
            # Create a temporary directory to store the TypeScript file
            with tempfile.TemporaryDirectory() as temp_dir:
                ts_file_path = os.path.join(temp_dir, "script.ts")
                js_file_path = os.path.join(temp_dir, "script.js")

                # Write the TypeScript code to a file
                with open(ts_file_path, "w") as ts_file:
                    ts_file.write(self.typescript_code)

                # Compile the TypeScript code to JavaScript
                compile_result = subprocess.run(
                    ["tsc", ts_file_path, "--outFile", js_file_path],
                    capture_output=True,
                    text=True
                )

                # Check for compilation errors
                if compile_result.returncode != 0:
                    return f"TypeScript compilation error:\n{compile_result.stderr}"

                # Run the compiled JavaScript code using Node.js
                run_result = subprocess.run(
                    ["node", js_file_path],
                    capture_output=True,
                    text=True
                )

                # Check for runtime errors
                if run_result.returncode != 0:
                    return f"Runtime error:\n{run_result.stderr}"

                # Return the output of the executed code
                return run_result.stdout

        except Exception as e:
            return f"An error occurred: {str(e)}"