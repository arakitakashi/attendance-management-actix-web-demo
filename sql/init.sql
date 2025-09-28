-- sql/init.sql
CREATE
EXTENSION IF NOT EXISTS "uuid-ossp";

-- 従業員テーブル
CREATE TABLE employees
(
    id            UUID PRIMARY KEY         DEFAULT uuid_generate_v4(),
    employee_code VARCHAR(50) UNIQUE NOT NULL,
    name          VARCHAR(100)       NOT NULL,
    email         VARCHAR(255),
    department    VARCHAR(100),
    created_at    TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at    TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 勤怠記録テーブル
CREATE TABLE attendance_records
(
    id             UUID PRIMARY KEY         DEFAULT uuid_generate_v4(),
    employee_id    UUID NOT NULL REFERENCES employees (id),
    work_date      DATE NOT NULL,
    clock_in_time  TIMESTAMP WITH TIME ZONE,
    clock_out_time TIMESTAMP WITH TIME ZONE,
    work_duration INTERVAL,
    status         VARCHAR(20)              DEFAULT 'working',
    created_at     TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at     TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    UNIQUE (employee_id, work_date)
);

-- インデックス
CREATE INDEX idx_attendance_records_employee_date ON attendance_records (employee_id, work_date);
CREATE INDEX idx_employees_code ON employees (employee_code);

-- テストデータ
INSERT INTO employees (employee_code, name, email, department)
VALUES ('ij09080022', '荒木太郎', 'test@company.com', '開発部'),
       ('1009080022', '荒木花子', 'test2@company.com', '営業部');