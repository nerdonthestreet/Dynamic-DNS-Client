# Dynamic DNS Client

This is a simple dynamic DNS client for use in Nerd on the Street infrastructure.

When the binary (compiled from the Rust project) is run, it calls a public API to check the calling machine's public IP address. It then checks a specified Linode DNS entry and, if necessary, updates it to match.

A systemd timer/unit pair are included for scheduling. The default timer runs every 5 minutes.

## Supporting development

If you find this utility useful, you can support development by joining the Nerd Club at [https://nerdclub.nots.co](nerdclub.nots.co).

If you want to contribute to development, you can open merge requests on GitLab (primary) or pull requests on GitHub (mirror). Commits from accepted GitHub PRs will be manually merged into GitLab with the authorship information intact.
