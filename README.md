quick_fox_status
================

Quick Fox Status: version 0.1.0
-----------
#### ** For changes see** [changelog](CHANGELOG.md).

## Quick Fox Status
-------

- [What is it](#what-is-it)
- [Requirement](#requirement)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Licence](#licence)
- [Author](#author)

## What is it
-------

This is a simple monitor addon for zabbix, that i use in script to give a status back that can be pulled by zabbix, using the zabbix agent2
(and probable also zabbix agent, but i only tried agent2)

For example you can use it in a script that needs to run hourly to give the status back of the script like this

```bash
if script is ok; then
    /usr/local/sbin/quick_fox_status add-service -s "myscript" --valid 2h --status=ok
else
    /usr/local/sbin/quick_fox_status add-service -s "myscript" --valid 2h --status=critical
fi
```

This is my first  rust program, so i want to give a lot of thanks to chagtp and claud.ai for helping with some of the code.


## Requirements
-------

- zabbix agent2 (if you want to use the zabbix discovery)
- rust installation (for ubuntu it was: sudo snap install --classic rustup && rustup default stable)
- make (sudo apt-get install make)


## Installation
-------

- git clone the repo to your server
- cd 'to repo dir'
- make install_suid   (for installation with suid so that all data is written as user zabbix)
- make install        (all data is written as user that runs quick_fox_status)

Add config to agent2.conf

```bash
# vim /etc/zabbix/zabbix_agent2.conf

### add config for discovery and check
UserParameter=custom.rdv_quick_fox_status.discovery,/usr/local/bin/quick_fox_status discovery
UserParameter=custom.rdv_quick_fox_status.check[*],/usr/local/bin/quick_fox_status discovery-check --name "$1"
```

Restart zabbix2 agent

```bash
# systemctl restart zabbix-agent2.service
```


## Configuration
-------

You can optional change the path where json config are and the default host name

The config file exists of

```
[config]
host = "myhost"
qfs_path = "/var/spool/quick_fox_status"
```

The following path's are posible

- /etc/quick_fox_status/config
- $HOME/.config/quick_fox_status/config
- <current_dir>/.quick_fox_status.toml


## Usage
-------

```bash
- quick_fox_status add-service  --service <service> --valid <valid_time> --status [ok, critical, unknown]
- quick_fox_status list-services                    # list all services with current status
- quick_fox_status status --name "<name>"           # get status of a service name
- quick_fox_status delete-service  --name "<name>"  # delete service # not implemented yet, just delete the json file in qfs_path
```

And for zabbix

```bash
quick_fox_status discovery                        # list services in a format for zabbix discovery, tested with zabbix 7.2
quick_fox_status discovery-check --name "<name>"  # check status of check
```

## License
-------

MIT license


## Author
-------

Richard de Vos

