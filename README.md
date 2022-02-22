# kv

A simple key-value store. Helps you in storing urls and other misc.

## Usage 
```
kv set es_url https://prd.es.hyperngn.com
# ^ 0 exit code and no output

kv get es_url
# ^ 0 exit code with following output on  
# https://prd.es.hyperngn.com

kv del es_url
# ^ 0 exit code and no output

kv list
# ^ 0 exit code with following output on  
#es_url\thttps://prd.es.hyperngn.com
```
