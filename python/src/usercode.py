def handle(taskId, data):
    data["python"] = "Hello, world!"
    data["pythonTaskId"] = taskId
    return data