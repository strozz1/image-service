# Media service
This is the GitHub project for the media service.
> Service still on development

This service is one of the many microservices related to the main project: [Blog](https://github.com/strozz1/Blog).
This project is manager for all the media files related to the Blog service, such as images, videos, gifs,...

We are going to explain the architecture of the service and its design and how the flow will go.
The service contains a few endpoints for consuming the API to access and manage all media files. 
In this file, we will be showing you how to consume the API in a proper way and what results tu expect from it. 

We will also be explaining all the different type of errors you could get and if its possible to fix them.
There will be examples for how to consume every endpoint correctly.


# Principal flow
We will discuss the principal flow of the service and how it works.

<img width="1085" alt="image" src="https://github.com/strozz1/image-service/assets/64174178/2b0dea44-883f-437b-a7dc-57c9dc416f7d">


> The image represents the flow of a client requesting a post action.

1. The client (consumer of API) sends a request for uploading file.
2. The server sends the file to a message queue which is listening for incoming messages. It stores them in the queue waiting for the consumers to take it.
3. The multimedia service is looking an the multimedia Queue and when a new message arrives, one consumer will take the message and remove it from the queue after retrieving the file.
4. The server saves the file info in the database with id and path information.
5. The server saves the file to the service storage.
6. The service continue to listen for incoming calls.

We are using a queue in this post operation because we want to apply some reliability on our system. By using an external queue, we ensure that we dont loose media files if the multimedia service is down. This way, the queue saves the data for when the service is ready again and the consumers are ready for fetching the stored messages (in this case binary files).

The architecture queue we are using is RabbitMQ. 

This way when the client ask to upload a file, the main server does not have to wait until the file is upload. It will send the file to the queue and continue doing other tasks.
This saves huges amounts of time on computation and in executing requests.

# Endpoints 
There are few endpoints for consuming the API and they will be discussed and explained.

|Endpoints|Type| Response Type
|--|--|--|
| /api/find | get/ | media file
| /api/upload | post/ | json
| /api/update | post/ | json
| /api/list | get/ | media file

## Endpoint - /api/find
**- Http Get operation**

This endpoint requires a parameter, the **id** of the file you want to retrieve.
>The id is a random generated string.

This parameter is required in the url, as showed above

    http://server.address/api/find?id=id_string


The server will search for this id in the database and get the path of the file and provide such file as a response.
### Http responses

The endpoint could have different responses depending on the success of the operation.
When the response is an error, the client will receive a json in the body response.
This are the few that can occur:

 - **200 /Ok** : The operation is successful and the client got the file.
 - **400 /BadRequest** : there was an error on the request, either didn't had a id parameter or malformed url.
 - **404 /NotFound** : the file or id was not found
 - **500 /Internal** : there was an internal server error
 
## Endpoint - /api/upload
**-Http post operation**

This endpoint flow is the one we explained earlier in the readme file.

The file is handed by the client in the /post http request
> The max file size the server is allowed to save is 50 Mb.
### Http responses
The endpoint could have different responses depending on the success of the operation.
When the response is an error, the client will receive a json in the body response and the reason.
This are the few that can occur:
 - **200 /Ok** : The server sent the file to the multimedia queue.
 - **400 /BadRequest** : there was an error with the file or not file was added.
 - **406 /NotAcceptable** :  the file provided is not allowed
 - **500 /Internal** : there was an internal server error

