# dd-unqueried-metrics
Rust program that uses the Datadog API to list your Datadog unqueried metrics.

The program uses the following [API Endpoint](https://docs.datadoghq.com/api/latest/metrics/#get-a-list-of-metrics): https://api.datadoghq.com/api/v2/metrics

The csv generated will contain your custom metrics that have not been queried for the past 2 weeks(current api limitation).

## Installation

If you have [Cargo installed](https://doc.rust-lang.org/cargo/getting-started/installation.html) you can run the following command to compile and install the binary in your PATH:

`cargo install --path .`

If you do not have Cargo installed, you can download the Binary from the releases page and add it to your PATH:
```
cp dd-unqueried-metrics /usr/local/bin/dd-unqueried-metrics
chmod +x /usr/local/bin/dd-unqueried-metrics
```
Then, reload/restart your terminal session

## Usage

In order to run the program you will need a [Datadog API Key and Application Key](https://docs.datadoghq.com/account_management/api-app-keys/).

You can then run the program like so:

`dd-unqueried-metrics --api-key <DD_API_KEY> --app-key <DD_APP_KEY>`

This also works:

`dd-unqueried-metrics --api-key=<DD_API_KEY> --app-key=<DD_APP_KEY>`

You can also run the program without the key parameters and the CLI will prompt you for it:

```
$dd-unqueried-metrics
Usage: ./dd-unqueried-metrics --api-key=<API_KEY> --app-key=<APP_KEY>
api_key and app-key parameters not provided, prompting user...

Please enter your Datadog API Key:
...
Please enter your Datadog Application Key:
...
```

The binary will generate a CSV file in your current directory:
`unqueried-metrics-DD-MM-YYYY`
