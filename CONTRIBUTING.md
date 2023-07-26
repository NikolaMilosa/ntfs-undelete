# Contributing to ntfs-undelete

Welcome to ntfs-undelete! We are excited to have you contribute to our project. By following this guide, you can help us improve the library and make it even better.

## Table of Contents

1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
    - [Fork the Repository](#fork-the-repository)
    - [Clone the Repository](#clone-the-repository)
3. [Making Changes](#making-changes)
4. [Testing](#testing)
5. [Submitting Pull Requests](#submitting-pull-requests)
6. [Contact](#contact)
7. [Acknowledgments](#acknowledgments)

## Introduction

ntfs-undelete is a binary for recovering deleted files from NTFS.

We welcome contributions from everyone, whether you're a seasoned developer or just getting started with open-source. You can contribute in many ways, such as:

- Fixing bugs
- Adding new features
- Improving documentation
- Enhancing test coverage
- Reporting issues
- Providing feedback

## Getting Started

To get started with contributing to ntfs-undelete, follow the steps below:

### Fork the Repository

Click the "Fork" button at the top right corner of the repository to create your own fork.

### Clone the Repository

Clone your forked repository to your local development environment:

```bash
git clone https://github.com/your-username/ntfs-undelete.git
cd ntfs-undelete
```

Replace `your-username` with your GitHub username.

## Making Changes

Now you can make changes to the codebase. Follow these best practices:

- Create a new branch for each contribution:

```bash
git checkout -b your-branch-name
```

- Write clear and concise commit messages.
- Make atomic commits for logically separate changes.
- Make sure your code is formatted according to the project's style guidelines.

```bash
cargo clippy
```

## Testing

We highly value the quality of our codebase, and tests play a crucial role in ensuring that quality. Before submitting your changes, run the test suite:

```bash
cargo test
```

Make sure all tests pass successfully.

## Submitting Pull Requests

Once you've made the necessary changes and ensured the tests pass, it's time to submit your contribution:

1. Push your changes to your forked repository:

```bash
git push origin your-branch-name
```

2. Go to the GitHub repository page for ntfs-undelete.
3. Click on the "Pull Requests" tab and then click "New Pull Request."
4. Select your branch from the dropdown and provide a descriptive title and summary of your changes.
5. If your pull request addresses any issues, reference them using the following syntax: "Fixes #issue_number."
6. Submit the pull request, and it will be reviewed by our team.

## Contact

If you have any questions or need further assistance, you can reach out to us via the following channels:

- GitHub Issues: [ntfs-undelete Issues](https://github.com/NikolaMilosa/ntfs-undelete/issues)

## Acknowledgments

We would like to express our gratitude to all contributors who help make ntfs-undelete better with their valuable contributions.

Thank you for being a part of our community and happy contributing!