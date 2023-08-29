#!/bin/bash

# Caminho para a aplicação
app_path=$PWD"/target/release/blckpp"
# Nome do serviço
service_name="blckpp"

# Conteúdo do arquivo de serviço
service_content="[Unit]
Description=BLCKPP
After=network.target

[Service]
Type=simple
ExecStart=$app_path
Restart=always

[Install]
WantedBy=default.target"

# Caminho para a pasta de serviços do Systemd
systemd_dir="/etc/systemd/system"

# Nome completo do arquivo de serviço
service_file="$systemd_dir/$service_name.service"

# Escrever o conteúdo do serviço no arquivo
echo "$service_content" | sudo tee $service_file

# Recarregar o Systemd para reconhecer o novo serviço
sudo systemctl daemon-reload

# Habilitar o serviço para iniciar na inicialização
sudo systemctl enable $service_name

# Iniciar o serviço
sudo systemctl start $service_name
