const { Command } = require("commander");
const os = require('os');
const boxen = require('boxen');
const chalk = require("chalk");

module.exports = new Command('pcinfo')
  .description('show informations of computer')
  .action(showCPU)

function showCPU() {
  console.log(
    boxen(
      `ARCH: ${os.arch()}\nPLATFORM: ${os.platform()}\nVERSION: ${os.release()}\nUSERNAME: ${
        os.userInfo().username
      }\nUID: ${os.userInfo().uid}\nSHELL: ${os.userInfo().shell}\nHOMEDIR: ${
        os.userInfo().homedir}\nGID: ${os.userInfo().gid}`,
      {
        borderColor: "magenta",
        align: "left",
      }
    )
  )
}