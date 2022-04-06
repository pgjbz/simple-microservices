package dev.pgjbz.order.domain.model;

import java.time.LocalDateTime;
import java.util.UUID;

import com.fasterxml.jackson.annotation.JsonProperty;

import dev.pgjbz.order.domain.enums.Status;

public record Order(
    UUID uuid,
    String name,
    String email,
    String phone,
    @JsonProperty(value = "product_id")
    UUID productId,
    Status status,
    @JsonProperty(value = "created_at")
    LocalDateTime createdAt
){}
