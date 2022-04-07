package dev.pgjbz.order.configuration;

import org.springframework.amqp.core.Binding;
import org.springframework.amqp.core.BindingBuilder;
import org.springframework.amqp.core.Queue;
import org.springframework.amqp.core.TopicExchange;
import org.springframework.beans.factory.annotation.Qualifier;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class RabbitMQConfig {

    @Bean
    public Queue checkoutQueue(@Value("${spring.rabbitmq.queues.checkout}") String queue) {
        return new Queue(queue, false);
    }

    @Bean
    public TopicExchange checkoutExchange(@Value("${spring.rabbitmq.exchanges.checkout}") String exchange) {
        return new TopicExchange(exchange);
    }

    @Bean
    public Binding binding(@Qualifier("checkoutQueue") Queue queue,
            @Qualifier("checkoutExchange") TopicExchange exchange,
            @Value("${spring.rabbitmq.prefixes.checkout}") String prefix) {
        return BindingBuilder.bind(queue).to(exchange).with(prefix);
    }

}
