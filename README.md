# Installation

Run the following command to install in your local Cargo set of installed binary crates. Type `cargo install --help` for more in-depth description of other installation options.

>   $ cargo install --git https://github.com/saibatizoku/benita-commander.git

# Usage

For help:
>   $ benita-commander -h
>   $ benita-commander conductivity <SUBCOMMAND> -h
>   $ benita-commander ph <SUBCOMMAND> -h
>   $ benita-commander temperature <SUBCOMMAND> -h

## REP servers

>   $ benita-commander conductivity rep -h
>   $ benita-commander ph rep -h
>   $ benita-commander temperature rep -h

>   $ benita-commander conductivity rep tcp://127.0.0.1:7777 /dev/i2c-0 77

>   $ benita-commander ph rep tcp://127.0.0.1:7778 /dev/i2c-0 78

>   $ benita-commander temperature rep tcp://127.0.0.1:7779 /dev/i2c-0 79

## REQ clients

>   $ benita-commander conductivity req -h
>   $ benita-commander ph req -h
>   $ benita-commander temperature req -h

For interactive mode (type `q` or `quit` to exit):
>   $ benita-commander conductivity req tcp://127.0.0.1:7777
>   conductivity>> [ENTER COMMAND]

>   $ benita-commander ph req tcp://127.0.0.1:7778
>   ph>> [ENTER COMMAND]

>   $ benita-commander temperature req tcp://127.0.0.1:7779
>   temperature>> [ENTER COMMAND]

For batch mode:
>   $ benita-commander conductivity req tcp://127.0.0.1:7777 -c CMD CMD CMD

>   $ benita-commander ph req tcp://127.0.0.1:7778 -c CMD CMD CMD

>   $ benita-commander temperature req tcp://127.0.0.1:7779 -c CMD CMD CMD
