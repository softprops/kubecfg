# kubecfg [![Build Status](https://travis-ci.org/softprops/kubecfg.svg?branch=master)](https://travis-ci.org/softprops/kubecfg) [![Coverage Status](https://coveralls.io/repos/github/softprops/kubecfg/badge.svg?branch=master)](https://coveralls.io/github/softprops/kubecfg?branch=master)

Kubecfg is an interface for reading and representing kubernetes config files in rustlang. Kubernetes configuration files store information needed to [authenticate across clusters](http://kubernetes.io/docs/user-guide/kubeconfig-file/). This is essential information if you wish to communicate with the kubernetes REST api.

## usage

Typically interaction with `kubectl` will result in a config file persisted to disk at a standard location on disk. To read that configuration information use `from_std_path()`

```rust
extern crate kubecfg;

use kubecfg::Config;

fn main() {
    if let Ok(cfg) = Config::from_std_path() {
        println!("current context is {}", cfg.current_context)
    }
}
```

Doug Tangren (softprops) 2016
