# GoSub: Gateway to Optimized Searching and Unlimited Browsing

This repository holds the GoSub HTML5 parser/tokenizer. It is a standalone library that can be used by other projects
but will ultimately be used by the GoSub browser. See the [About](#about) section for more information.

```
                       _     
                      | |    
  __ _  ___  ___ _   _| |__  
 / _` |/ _ \/ __| | | | '_ \ 
| (_| | (_) \__ \ |_| | |_) |
 \__, |\___/|___/\__,_|_.__/ 
  __/ |  The Gateway to                    
 |___/   Optimized Searching and 
         Unlimited Browsing                    
```


## About

This repository is part of the GoSub browser project. Currently, there is only a single component/repository (this one), 
but the idea will be that there are many other components that, as a whole, make up a full-fledged browser. Each of the 
components can probably function as something standalone (ie, html5 parser, CSS parser, etc.).

In the future, this component (HTML5 parser) will receive a stream of bytes through an API and output a stream of
events. The next component will consume the events, and so on, until we can display something in a window/user 
agent. This could be a text-mode browser, but the idea is to have a graphical browser.


## Status

> This project is in its infancy. There is no browser you can use yet.

This is a work in progress. The current status is that the parser can parse a few HTML5 documents, but it is far from
ready. The main goal is to be able to parse correctly all the tests in the html5lib-tests repository 
(https://github.com/html5lib/html5lib-tests). 

Our goal at the moment is to research as much as possible and to setup proof-of-concepts in order to gain more 
understanding in the field of browsers. We are not trying to create a full-fledged browser at the moment, but it will
be our ultimate goal. 

## How to build

This project uses cargo (https://doc.rust-lang.org/cargo/). To build the project, run:

```bash
cargo build
```

This will create the following binaries and libs:

| File           | Type | Description                       |
|----------------|------|-----------------------------------|
| gosub-engine   | lib  | The actual html5 parser/tokenizer |
| parser_test    | bin  | A test suite for the parser       |

### Gosub-engine

This is the actual html5 parser/tokenizer. It is a library that can be used for other projects. It is not a standalone
project but can be incorporated into other projects. 

### Parser_test

This is a test suite for the parser. It is not a standalone project. It is used by the gosub-engine project. You need 
to specify the directory to the html5lib-test to run, or it will use the default one (./html5lib-tests).

```bash
$ parser_test 
```