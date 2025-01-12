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

Run the following command on development mode:

```bash
cargo r -- \
--pr-number <PR Number to Verify> \
--pr-author <PR Author GitHub's Username> \
--github-token <GitHub Token>
```

## License

This project is licensed under the MIT license.

[1]: https://docs.github.com/en/actions/writing-workflows/choosing-what-your-workflow-does/accessing-contextual-information-about-workflow-runs#github-context
