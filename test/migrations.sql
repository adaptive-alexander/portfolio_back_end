CREATE TABLE hedgenordic_fund_performance (
eop_date TIMESTAMP NOT NULL,
name VARCHAR(50) NOT NULL,
perf_monthly FLOAT(53) NOT NULL,
PRIMARY KEY (eop_date, name));

INSERT INTO hedgenordic_fund_performance VALUES ('2022-11-26', 'Test_Fund', '0.22');

-- docker exec -it postgres-Mhx1 psql -U postgres
