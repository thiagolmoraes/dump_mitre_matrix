@echo off
setlocal

REM Define o caminho do banco de dados e do SQLite
set DATABASE_PATH=database.db
set SQLITE_PATH=sqlite3.exe

REM Executa comandos SQLite para deletar dados e otimizar o banco de dados
%SQLITE_PATH% %DATABASE_PATH% "DELETE FROM tactics; VACUUM;"
%SQLITE_PATH% %DATABASE_PATH% "DELETE FROM techniques; VACUUM;"
%SQLITE_PATH% %DATABASE_PATH% "DELETE FROM tactic_technique; VACUUM;"

echo Dados das tabelas 'tactics', 'techniques' e 'tactic_technique' deletados, índices resetados e táticas MITRE ATT&CK inseridas.
pause
