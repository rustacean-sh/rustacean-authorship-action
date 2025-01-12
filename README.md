<h1 align=center>
  Rustacean.sh Authorship Action
</h1>
<p align=center>
  Action to verify Rustacean Authenticity on PRs modifying a Rustacean file.
</p>

## Motivation

Provide an automated way to verify the authenticity of a Rustacean file in a PR.
This is achieved by checking the authorship of the file and comparing it to the
PR author.

This Action leverages the GitHub API and the [GitHub Actions Context][1] to verify
the authenticity of the Rustacean file.

## Development

For convenience a _mock_ server is provided which is used to simulate the GitHub API.

Run the following command to start the server:

```bash
cargo r --bin mock
```

> The server listens on port `3000` by default.

Run the following command on development mode:

```bash
GITHUB_BASE_URL="http://localhost:3000/repos/rustacean-sh/rustacean.sh" \
cargo r --bin rustacean-authorship-action -- \
--pr-number <PR Number to Verify> \
--pr-author <PR Author GitHub's Username> \
--github-token <GitHub Token>
```

> Static assets on the `mock` server are located at `./fixtures`, you can modify
> them to test different scenarios. Default values are:
> `--pr-number 22`
> `--pr-author EstebanBorai`
> `--github-token 1234`

## License

This project is licensed under the MIT license.

[1]: https://docs.github.com/en/actions/writing-workflows/choosing-what-your-workflow-does/accessing-contextual-information-about-workflow-runs#github-context
