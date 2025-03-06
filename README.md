# `hivemind`

`hivemind` is a tool targeting individuals that need highly extensible workflows for task management.

## goals

- integrate with any other tool
  `hivemind` architecture not only allows for new integrations to be added, the user is able to extend it with numerous forms (shell/python/javascript/etc scripts, watching databases (postgres, mysql, sqlite, etc), etc.)
- privacy-focused and self-hostable
  private and personal tasks are expected, hosting the project yourself is better than using someone else's "cloud"

## non-goals

- become X (JIRA, taskwarrior, etc)
  there's no point in reinventing the wheel, even if it means rewriting it in rust. `hivemind` aims to give new features, a better user experience and integration with existing tools
- cloud service
  I can't see a way to profit on cloud subscriptions without going bankrupt because of power-users at this moment

## more information

- [LICENSE](LICENSE)
- [TODO.md](TODO.md)
