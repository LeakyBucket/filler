# Filler

Filler is a dumb tool for putting things into things.  Say for some awful reason you have a file and you need to put values into it that live in different locations but you don't want them just sitting in the file (think template) then you could use this thing to do that.

Are you stuck in a paper sack that you can't get out of?  If so read on...

## Sources

* Env
* SSM Parameter Store (AWS)
* Custom definitions

Let's start with the first two as they are the most straight forward:

Like most things Filler can pull data out of it's environment, in order to specify a value that should be found in the environment use the following format (unless you've changed the format): `{{ env:KEY }}`.  Whenever that pattern is found it will be replaced with whatever is found in that environment variable.  If there is no value for that env var the key will remain in place (maybe the next person/context is also in a bad spot but they have that data).

The AWS SSM Parameter store is similar, unless you have changed the defaults simply use `{{ ssm:RDSPassword }}` to pull data out of Parameter Store.  Similar to the env behavior if there is no value then the pattern will remain in the file for potential future processing.

The third source is kind of a gaint bucket of "whatever".  Filler supports the definition of "custom" commands for retrieving data.  A custom command is defined as follows:

```
  "commands": {
    "cat": {
      "command": "cat",
      "position": "Last"
    },
    "credstash": {
      "command": "credstash",
      "parameters": [
        "get"
      ],
      "position": "Last"
    }
  }
```

In the case of a command there are three possible attributes:

* command - the actual command to invoke
* parameters - a list of arguments to provide the command
* position - the position in the arg list where the template value should be placed (First or Last)

Filler will execute the command and then replace the marker with the output of the command.  To mark the position for a command simply use the following format `{{ credstash:rds.user.password }}`.

## Configuration

Filler does expect a configuration file which is currently `JSON` format:

```
{
  "placeholder": {
    "opening": "[[",
    "closing": "]]",
    "separator": ":"
  },
  "commands": {
    "credstash": {
      "command": "credstash",
      "flags": [
        "get"
      ],
      "position": "Last"
    }
  }
}
```

As alluded to above in a few places, it is possible to customize the templating format.  You have control over the opening pattern, separator and closing pattern.

## Trouble?

Are you having trouble/encountering issues?  Don't worry I believe you, after all I wrote this.  Please feel free to submit PRs or open an Issue.