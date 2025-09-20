module github.com/distsys-course/grpc-messenger/client

go 1.23

replace github.com/distsys-course/grpc-messenger/grpc => ../proto

require (
	github.com/distsys-course/grpc-messenger/grpc v0.0.0-00010101000000-000000000000
	github.com/gin-gonic/gin v1.10.1
	github.com/golang/protobuf v1.5.4
	google.golang.org/grpc v1.75.0
)
