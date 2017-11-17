# Installation

Run the following command to install in your local Cargo set of installed binary crates. Type `cargo install --help` for more in-depth description of other installation options.

__NOTE:___ It's necessary to install with `nightly` due to the use of still unstable features.

>   $ rustup run nightly cargo install --git https://github.com/saibatizoku/benita-commander.git

# Usage

For help:
>   $ benita-commander -h

>   $ benita-commander conductivity <SUBCOMMAND> -h
  
>   $ benita-commander ph <SUBCOMMAND> -h
  
>   $ benita-commander temperature <SUBCOMMAND> -h

## REP servers

### Help

>   $ benita-commander conductivity rep -h

>   $ benita-commander ph rep -h

>   $ benita-commander temperature rep -h


### Starting a REP server with command-line arguments

>   $ benita-commander conductivity rep tcp://127.0.0.1:7777 /dev/i2c-0 77

>   $ benita-commander ph rep tcp://127.0.0.1:7778 /dev/i2c-0 78

>   $ benita-commander temperature rep tcp://127.0.0.1:7779 /dev/i2c-0 79

### Starting a REP server with ENV variables

#### Conductivity

>   $ export CONDUCTIVITY_REP_URL= tcp://127.0.0.1:7777
>
>   $ export CONDUCTIVITY_REP_PATH="/dev/i2c-0"
>
>   $ export CONDUCTIVITY_REP_ADDRESS=77
>
>   $ benita-commander conductivity rep

#### pH

>   $ export PH_REP_URL= tcp://127.0.0.1:7778
>
>   $ export PH_REP_PATH="/dev/i2c-0"
>
>   $ export PH_REP_ADDRESS=78
>
>   $ benita-commander ph rep

#### Temperature

>   $ export TEMPERATURE_REP_URL= tcp://127.0.0.1:7779
>
>   $ export TEMPERATURE_REP_PATH="/dev/i2c-0"
>
>   $ export TEMPERATURE_REP_ADDRESS=79
>
>   $ benita-commander temperature rep

## REQ clients

### Help

>   $ benita-commander conductivity req -h

>   $ benita-commander ph req -h

>   $ benita-commander temperature req -h

### Starting a REQ client with interactive mode

Press `q` or `quit` to exit the client.

>   $ benita-commander conductivity req tcp://127.0.0.1:7777
>
>   conductivity>> [ENTER COMMAND]

>   $ benita-commander ph req tcp://127.0.0.1:7778
>
>   ph>> [ENTER COMMAND]

>   $ benita-commander temperature req tcp://127.0.0.1:7779
>
>   temperature>> [ENTER COMMAND]

### Starting a REQ client with batch mode

>   $ benita-commander conductivity req tcp://127.0.0.1:7777 -c CMD CMD CMD

>   $ benita-commander ph req tcp://127.0.0.1:7778 -c CMD CMD CMD

>   $ benita-commander temperature req tcp://127.0.0.1:7779 -c CMD CMD CMD

### Using with ENV variables instead of command-line arguments

#### Conductivity

>   $ export CONDUCTIVITY_REQ_URL="tcp://127.0.0.1:7777"

>   $ benita-commander conductivity req

>   $ benita-commander conductivity req -c CMD CMD CMD

#### pH

>   $ export PH_REQ_URL="tcp://127.0.0.1:7778"

>   $ benita-commander ph req

>   $ benita-commander ph req -c CMD CMD CMD

#### Temperature

>   $ export TEMPERATURE_REQ_URL="tcp://127.0.0.1:7779"

>   $ benita-commander temperature req

>   $ benita-commander temperature req -c CMD CMD CMD
