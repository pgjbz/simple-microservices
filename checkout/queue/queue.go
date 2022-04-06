package queue

import (
	"fmt"
	"github.com/streadway/amqp"
	"os"
)

func Connect() *amqp.Channel {
	dns := "amqp://" + os.Getenv("RABBITMQ_USERNAME") + 
		":" + os.Getenv("RABBITMQ_PASSWORD") +
		"@" + os.Getenv("RABBITMQ_HOST") +
		":" + os.Getenv("RABBITMQ_PORT") + "/"
	conn, err := amqp.Dial(dns)

	if err != nil {
		panic(err.Error())
	}

	channel, err := conn.Channel()

	if err != nil {
		panic(err.Error())
	}
	return channel
}

func Notify(payload []byte, exchange string, routingKey string, ch *amqp.Channel) {

	err := ch.Publish(
		exchange,
		routingKey,
		false,
		false,
		amqp.Publishing{
			ContentType: "application/json",
			Body:        []byte(payload),
		})

	if err != nil {
		panic(err.Error())
	}

	fmt.Println("Message sent")
}