package dev.pgjbz.order.configuration;

import org.springframework.amqp.core.Binding;
import org.springframework.amqp.core.BindingBuilder;
import org.springframework.amqp.core.Queue;
import org.springframework.amqp.core.TopicExchange;
import org.springframework.amqp.rabbit.connection.ConnectionFactory;
import org.springframework.amqp.rabbit.listener.SimpleMessageListenerContainer;
import org.springframework.amqp.rabbit.listener.adapter.MessageListenerAdapter;
import org.springframework.amqp.support.converter.Jackson2JsonMessageConverter;
import org.springframework.beans.factory.annotation.Qualifier;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.context.annotation.Primary;

import dev.pgjbz.order.handles.ReceiveMessageHandler;

@Configuration
public class RabbitMQConfig {

    @Bean
    public Queue checkoutQueue(@Value("${spring.rabbitmq.queues.checkout}") String queue) {
        return new Queue(queue, true);
    }

    @Bean
    public TopicExchange checkoutExchange(@Value("${spring.rabbitmq.exchanges.checkout}") String exchange) {
        return new TopicExchange(exchange);
    }

    @Bean

    public Binding binding(@Qualifier("checkoutQueue") Queue queue,
            @Qualifier("checkoutExchange") TopicExchange exchange,
            @Value("${spring.rabbitmq.prefixes.checkout}") String prefix) {
        return BindingBuilder.bind(queue).to(exchange).with("prefix");
    }

    @Bean
    public Jackson2JsonMessageConverter messageConverter() {
        return new Jackson2JsonMessageConverter();
    }

    @Bean
    public SimpleMessageListenerContainer checkoutContainer(ConnectionFactory connectionFactory,
                                             MessageListenerAdapter listenerAdapter,
                                             @Value("${spring.rabbitmq.queues.checkout}") String queue) {
        var container = new SimpleMessageListenerContainer();
        container.setConnectionFactory(connectionFactory);
        container.setQueueNames(queue);
        container.setMessageListener(listenerAdapter);
        return container;
    }

    @Bean
    @Primary
    public MessageListenerAdapter listenerAdapter(ReceiveMessageHandler receiver) {
        return new MessageListenerAdapter(receiver, "receiveMessage");
    }


}
