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

    @Bean(name = "checkoutQueue")
    public Queue checkoutQueue(@Value("${spring.rabbitmq.queues.checkout}") String queue) {
        return new Queue(queue, false);
    }


    @Bean(name = "checkoutExchange")
    public TopicExchange checkoutExchange(@Value("${spring.rabbitmq.exchanges.checkout}") String exchange) {
        return new TopicExchange(exchange);
    }

  

    @Bean(name = "checkoutBinding")
    public Binding checkoutBinding(@Qualifier("checkoutQueue") Queue queue,
            @Qualifier("checkoutExchange") TopicExchange exchange,
            @Value("${spring.rabbitmq.prefixes.checkout}") String prefix) {
        return BindingBuilder.bind(queue).to(exchange).with(prefix);
    }

    @Bean(name = "paymentQueue")
    public Queue paymentQueue(@Value("${spring.rabbitmq.queues.payment}") String queue) {
        return new Queue(queue, false);
    }

    @Bean(name = "paymentExchange")
    public TopicExchange paymentExchange(@Value("${spring.rabbitmq.exchanges.payment}") String exchange) {
        return new TopicExchange(exchange);
    }

    @Bean(name = "paymentBinding")
    public Binding paymentBinding(@Qualifier("paymentQueue") Queue queue,
            @Qualifier("paymentExchange") TopicExchange exchange,
            @Value("${spring.rabbitmq.prefixes.payment}") String prefix) {
        return BindingBuilder.bind(queue).to(exchange).with(prefix);
    }

    @Bean(name = "orderQueue")
    public Queue orderQueue(@Value("${spring.rabbitmq.queues.order}") String queue) {
        return new Queue(queue, false);
    }

    @Bean(name = "orderExchange")
    public TopicExchange orderExchange(@Value("${spring.rabbitmq.exchanges.order}") String exchange) {
        return new TopicExchange(exchange);
    }

    @Bean(name = "orderBinding")
    public Binding orderBinding(@Qualifier("orderQueue") Queue queue,
            @Qualifier("orderExchange") TopicExchange exchange,
            @Value("${spring.rabbitmq.prefixes.order}") String prefix) {
        return BindingBuilder.bind(queue).to(exchange).with(prefix);
    }
    
}
