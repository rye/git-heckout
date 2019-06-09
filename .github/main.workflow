workflow "Environment Printer" {
  on = "push"
  resolves = ["alpine"]
}

action "alpine" {
  uses = "docker://alpine:latest@769fddc7cc2f0a1c35abb2f91432e8beecf83916c421420e6a6da9f8975464b6"
  args = "env"
}
