# Filler

Filler is a dumb tool for putting things into things.  Say for some awful reason you have a file and you need to put values into it that live in different locations but you don't want them just sitting in the file (think template) then you could use this thing to do that.

Are you stuck in a paper sack that you can't get out of?  If so read on...

## Usage

```
Fills in config files with sensitive data

USAGE:
    filler [OPTIONS] <target>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <conf>    specify a config file
    -o, --output <out>     the output file to generate, if omitted the target file name will have .filled appended to it

ARGS:
    <target>    the file to be processed
```

## Example

First a maybe contrived example.  Let's say I need to get data from the environment, `SSM` and a snippet into the following file:

```ini
[Stuff]

env.thing={{ env:SUB }}
ssm.data={{ ssm:SSMTarget }}

{{ cat:snippet.ini }}
```

With the following configuration I could generate the desired file:

```json
{
  "commands": {
    "cat": {
      "command": "cat",
      "position": "Last"
    }
  }
}
```

## Sources

* Env
* SSM Parameter Store (AWS)
* Custom definitions

Let's start with the first two as they are the most straight forward:

Like most things Filler can pull data out of it's environment, in order to specify a value that should be found in the environment use the following format (unless you've changed the format): `{{ env:KEY }}`.  Whenever that pattern is found it will be replaced with whatever is found in that environment variable.  If there is no value for that env var the key will remain in place (maybe the next person/context is also in a bad spot but they have that data).

The AWS SSM Parameter store is similar, unless you have changed the defaults simply use `{{ ssm:RDSPassword }}` to pull data out of Parameter Store.  Similar to the env behavior if there is no value then the pattern will remain in the file for potential future processing.

### Commands

The third source is kind of a gaint bucket of "whatever".  Filler supports the definition of "custom" commands for retrieving data.  A custom command is defined as follows:

```json
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

In the case of a command there are four possible attributes:

* command - the actual command to invoke
* parameters - a list of arguments to provide the command
* version - configuration for commands that support versioning
* position - the position in the arg list where the template value should be placed (First or Last)

#### Command

This is the actual executable to invoke.  The command either needs to be in the `PATH` or you should specify the absolute path to the command to run.  This string should not include any flags or arguments for the command.

Filler will execute the command and then replace the marker with the output of the command.  To mark the position for a command simply use the following format `{{ credstash:rds.user.password }}`.

#### Parameters

This is a list of arguments that should be given to the command to be executed.  When constructing this list just think of the full string you would usually enter as a space delimited list.  In other words if you wanted to invoke `ls -a -l` you would need to provide a parameter list like so:

```
"parameters": [
  "-a",
  "-l"
]
```

#### Version

If you are using a tool that supports versioning the version arguments should be specified separately in the config.  This is necessary in the event that you want or need to mix versioned and non-versioned placeholder in a file.  The version argument will only be included if the placeholder specifies a version.

For example, let's say you needed two values from credstash.  However you always want the current DB password but need a specific API Key value.  In this specific case there would be two placeholders looking something like this:

```
{{ credstash:prod.db.password }}
...
{{ credstash:prod.api_key:3 }}
```

These can both be handled by adding a version section to the `credstash` command configuration:

```
"version": {
  "flag": "-v",
  "format": "Disperate"
}
```

With the above config the versioned `credstash` placeholder will add `-v 3` to the argument list.

The version configuration is not very complicated.  It consists of a `flag` attribute and a `format` attribute.  The `format` attribute has two valid values `Disperate` and `Concatinate`.  If set to `Dispearate` the flag and the value will be separated with a space.  If set to `Concatinate` then they will be _joined_.

#### Position

There is also a `position` attribute that allows a bit of extra control over the shape of the generated command.  There are two legal values: `First` and `Last`.  This simply dictates whether the label in the placeholder will be added before or after the parameters when generating the command.

## Configuration

Filler does expect a configuration file which is currently `JSON` format:

```json
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