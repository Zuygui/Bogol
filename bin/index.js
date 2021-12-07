#! /usr/bin/env node
const { program } = require('commander');
const chalk = require('chalk');
const cellular = require('./cli');
const cpu = require('./cpu');
const { help } = require('./cpu');

program
  .version("1.0.0", '-v, --version', 'show version.')
  .name(chalk.green.bold("bogol"))
  .usage(chalk.red.bold(`<option> ${chalk.yellow("[args]")}`))
  .helpOption("-h, --help", "show help.")
  .addCommand(cpu)
  .action(help)

program.parse()