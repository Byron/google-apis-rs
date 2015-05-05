## Issues

All issues, may it be feature requests, or bugs, are treated as **problems**.
In any case it is important to be able to reproduce or understand it. To help in doing so, please state the following information:

* a description of the problem
* steps to reproduce the problem (if applicable)
* library version showing the problem (if applicable)

The emphasis on the term **problem** originates from the observed tendency of issues being more like a solution to a problem,
which is not stated in detail, making good solutions more difficult to achieve.

## Pull Requests

Feel free to make any kind of Pull Request - all contributions are welcome.

The only thing to keep in mind is the commit message format, which is described below.

### Commit Message Format

The format is taken directly from [angular.js][angular-contribution-guide], as is this text.

Each commit message consists of a **header**, a **body** and a **footer**.  The header has a special
format that includes a **type**, a **scope** and a **subject**:

```
<type>(<scope>): <subject>
<BLANK LINE>
<body>
<BLANK LINE>
<footer>
```

Any line of the commit message cannot be longer 100 characters! This allows the message to be easier
to read on github as well as in various git tools.

### Type
Must be one of the following:

* **feat**: A new feature
* **fix**: A bug fix
* **docs**: Documentation only changes
* **style**: Changes that do not affect the meaning of the code (white-space, formatting, missing
  semi-colons, etc)
* **refactor**: A code change that neither fixes a bug or adds a feature
* **perf**: A code change that improves performance
* **test**: Adding missing tests
* **chore**: Changes to the build process or auxiliary tools and libraries such as documentation
  generation
* **imp**: An improvement to an existing feature, which faces the user. For internal improvements,
  use the *refactor* type

### Scope
The scope could be anything specifying place of the commit change. For example `$location`,
`$browser`, `$compile`, `$rootScope`, `ngHref`, `ngClick`, `ngView`, etc...

### Subject
The subject contains succinct description of the change:

* use the imperative, present tense: "change" not "changed" nor "changes"
* don't capitalize first letter
* no dot (.) at the end

###Body
Just as in the **subject**, use the imperative, present tense: "change" not "changed" nor "changes"
The body should include the motivation for the change and contrast this with previous behavior.

###Footer
The footer should contain any information about **Breaking Changes** and is also the place to
reference GitHub issues that this commit **Closes**.

[angular-contribution-guide]: https://github.com/angular/angular.js/blob/master/CONTRIBUTING.md