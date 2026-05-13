-- Создание таблицы posts
CREATE TABLE posts (
                       id BIGSERIAL PRIMARY KEY,
                       title VARCHAR(255) NOT NULL,
                       content TEXT NOT NULL,
                       author_id BIGINT NOT NULL,
                       created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                       updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    -- Внешний ключ с каскадным удалением
                       CONSTRAINT fk_posts_author
                           FOREIGN KEY (author_id)
                               REFERENCES users(id)
                               ON DELETE CASCADE
);

-- Создание индексов для оптимизации запросов
CREATE INDEX idx_posts_author_id ON posts (author_id);
CREATE INDEX idx_posts_created_at ON posts (created_at);

-- Дополнительный индекс для сортировки по updated_at (полезно для "последние обновления")
CREATE INDEX idx_posts_updated_at ON posts (updated_at);