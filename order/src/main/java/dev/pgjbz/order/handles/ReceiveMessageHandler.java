package dev.pgjbz.order.handles;

import java.time.LocalDateTime;
import java.util.UUID;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;

import org.springframework.amqp.core.Message;
import org.springframework.amqp.rabbit.annotation.RabbitListener;
import org.springframework.amqp.rabbit.core.RabbitTemplate;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.data.redis.core.ReactiveValueOperations;
import org.springframework.stereotype.Component;

import dev.pgjbz.order.domain.enums.Status;
import dev.pgjbz.order.domain.model.Order;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;

@Slf4j
@Component
@RequiredArgsConstructor
public class ReceiveMessageHandler {

    private final ObjectMapper mapper;
    private final ReactiveValueOperations<String, Order> redisOps;
    private final RabbitTemplate rabbitTemplate;

    @Value("${spring.rabbitmq.exchanges.payment}")
    private String paymentExchange;
    @Value("${spring.rabbitmq.prefixes.payment}")
    private String paymentPrefix;

    @RabbitListener(queues = "${spring.rabbitmq.queues.checkout}")
    public void handleMessage(String message) {
        try {
            log.info("processing message {}", message);
            var order  = orderToSave(mapper.readValue(message, Order.class));
            redisOps.set(order.uuid().toString(), order).block();
            rabbitTemplate.send(paymentExchange, paymentPrefix, new Message(message.getBytes()));
        } catch (JsonProcessingException e) {
            log.error("error in operation {}", message);
        }
    }

    private Order orderToSave(Order order) {
        return new Order(UUID.randomUUID(), order.name(), order.email(), order.phone(), order.productId(),
                Status.PENDING, LocalDateTime.now());
    }

}
