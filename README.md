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

<img width="1085" alt="image" src="https://github.com/strozz1/image-service/assets/64174178/a67b04ac-f53d-4eef-bd04-ca01164a98f3">


> The image represents the flow of a client requesting an image by its id.

1.  The client (consumer of API) sends a request for uploading file, retrieve one, get ids...
2. The server looks in de database for all the info about this file the client is looking for. If the file does not exists, the server notifies the client about it.
3. The database retrieves the file data such as file id, path, and other metadata of the file.
4. The server searches the file with the path obtained in step 3.
5. The server obtains the file the client is looking for.
6. The file is retrieved by the client.

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

This endpoint uploads a media file into the server. First, the server creates a random id string for the media file and saves it into the database with all its information. After, it saves the file into the internal archives and returns the client a json with the operation result and the id generated.

The file is handed by the client in the /post http request
> The max file size the server is allowed to save is 50 Mb.
### Http responses
The endpoint could have different responses depending on the success of the operation.
When the response is an error, the client will receive a json in the body response and the reason.
This are the few that can occur:
 - **200 /Ok** : The operation is successful and the client got the id in the json body.
 - **400 /BadRequest** : there was an error with the file or not file was added.
 - **406 /NotAcceptable** :  the file provided is not allowed
 - **500 /Internal** : there was an internal server error

