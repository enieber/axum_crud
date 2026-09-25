# list task
GET {{host}}/tasks
HTTP 200

# create task
POST {{host}}/tasks
{
    "title": "dar Bom Dia"
}
HTTP 201

# update task
PATCH {{host}}/tasks/1
{
    "title":"Boa tarde"
}
HTTP 200

# delete task
DELETE {{host}}/tasks/4
HTTP 204
