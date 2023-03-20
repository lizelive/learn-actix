wget -O- https://aka.ms/install-vscode-server/setup.sh | sh
code-server serve-local --accept-server-license-terms --without-connection-token --host=127.0.0.1 --port=8000 --verbose --log trace --disable-telemetry


mutantdino.resourcemonitor
ms-vscode.remote-repositories
ms-python.python
VisualStudioExptTeam.vscodeintellicode


export PORT=8003;  sudo ssh -i ~/.ssh/id_rsa azureuser@20.190.19.41 -p 50000 -L 80:localhost:$PORT -- code-server serve-local --accept-server-license-terms --without-connection-token --host=127.0.0.1 --port=$PORT --verbose --log trace --disable-telemetry


