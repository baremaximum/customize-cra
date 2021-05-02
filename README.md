# customize-cra
A utility that helps automate the process of adding various tools to applications created with create-react-app.

**NOT READY FOR USE**

## usage

Run using the command:

```shell
cust-cra 
```

Use flags to specify what you want to add to the application.

## flags

### --cypress

Adds cypress and creates a `cypress` directory in project root. Also adds cypress scripts to `package.json`.

### --tailwind

Adds tailwind, and replaces react-scripts with craco in `package.json`.