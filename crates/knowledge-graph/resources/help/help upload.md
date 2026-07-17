Upload mock data to current graph instance
------------------------------------------
When the following command is entered, the system will print out a URL for you to upload
a JSON payload to the current graph instance.

Syntax
------
```
upload mock data
```

Upon receiving a HTTP POST request to the given URL, the JSON request payload will be used
as mock "input.body".

If you want to mock some input headers or the state machine, please use the "instantiate graph" command
before uploading.
