# dekube-cli
cli-app for dekube application written in rust

Cli app for dekube

user should be able to login to their dekube account 

user should be able to create dekube project 

user should be able to list their dekube projects 

user should be able to push dockerfile to dekube project 

user should be able to deploy dekube project

##commands
dekube login --email --password 

dekube create-app --name 

dekube list-app 

dekube upload --file --app 

dekube deploy --app

##todo
clean up code

###upload
compress before uploading 

ensure authenticated 

fetch secrets and upload key based on input and auth 

add app to upload command and upload per app 

confirm path contains dekube.config file parse and validate dekube.config file before uploading
