# kv
<a href="https://crates.io/crates/bash-kv">
    <img src="https://img.shields.io/crates/v/bash-kv.svg">
</a>

A simple key-value store. Helps you in storing urls and other misc.

## Usage 
```
kv set es_url https://prd.es.hyperngn.com
# ^ 0 exit code and no output

kv get es_url
# ^ 0 exit code with following output on  
# https://prd.es.hyperngn.com
# use this value to fetch data
curl -v "$(kv get es_url)"

kv del es_url
# ^ 0 exit code and no output

kv list
# ^ 0 exit code with following output on  
#es_url\thttps://prd.es.hyperngn.com
```
