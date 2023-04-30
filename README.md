# ddocs
```

      .o8        .o8
     "888       "888
 .oooo888   .oooo888   .ooooo.   .ooooo.   .oooo.o
d88' `888  d88' `888  d88' `88b d88' `"Y8 d88(  "8
888   888  888   888  888   888 888       `"Y88b.
888   888  888   888  888   888 888   .o8 o.  )88b
`Y8bod88P" `Y8bod88P" `Y8bod8P' `Y8bod8P' 8""888P'
```

## About
Simple cli tool to save and show your text docs

save docs with ddocs and feel relief
because it will compress your data so you can enjoy from more space

and i guess i should say its fast :)

this is how i save docs for you
![cat](https://github.com/Arian-p1/ddocs/blob/master/cat.png?raw=true)
![show-json-conf](https://github.com/Arian-p1/ddocs/blob/master/show-conf.png?raw=true)


## Install

just clone the repo and run
```sh
cargo build --release
mv target/release/ddocs /usr/bin/
```

Also it dosent work on windows

## Document
```
Usage: ddocs [OPTIONS]

Options:
  -s, --search <SEARCH>  search the topic you saved
  -a, --add <ADD>        add your new topic
  -c, --cat <CAT>        cat your topic
  -e, --edit <EDIT>      edit topic (its on TODO)
  -d, --delete <DELETE>  delete the topic
  -h, --help             Print help
  -V, --version          Print version
```
