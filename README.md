# FL0test - GRPC
``````
███████╗██╗      ██████╗ ████████╗███████╗███████╗████████╗         ██████╗ ██████╗ ██████╗  ██████╗
██╔════╝██║     ██╔═████╗╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝        ██╔════╝ ██╔══██╗██╔══██╗██╔════╝
█████╗  ██║     ██║██╔██║   ██║   █████╗  ███████╗   ██║   █████╗  ██║  ███╗██████╔╝██████╔╝██║
██╔══╝  ██║     ████╔╝██║   ██║   ██╔══╝  ╚════██║   ██║   ╚════╝  ██║   ██║██╔══██╗██╔═══╝ ██║
██║     ███████╗╚██████╔╝   ██║   ███████╗███████║   ██║           ╚██████╔╝██║  ██║██║     ╚██████╗
╚═╝     ╚══════╝ ╚═════╝    ╚═╝   ╚══════╝╚══════╝   ╚═╝            ╚═════╝ ╚═╝  ╚═╝╚═╝      ╚═════╝

``````

Example GRPC service for testing and validating various bits of GRPC functionality on the FL0 platform.

Running it is as simple as executing `cargo run` in the root directory.
By default, it will attempt to bind to port `8080` on `localhost` (`127.0.0.1`) however this behaviour can be overridden by setting the `PORT` and `ENVIRONMENT` variables.

Setting `ENVIRONMENT` to `PROD` will cause it to bind to `0.0.0.0` instead.
