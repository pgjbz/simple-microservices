package dev.pgjbz.order.handles;

import org.springframework.stereotype.Component;

import dev.pgjbz.order.domain.model.Order;
import lombok.extern.slf4j.Slf4j;

@Slf4j
@Component
public class ReceiveMessageHandler {
    
    public void receiveMessage(Order message) {
        log.info("{}", message);
    }

}
