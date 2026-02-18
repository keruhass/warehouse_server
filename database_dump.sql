--
-- PostgreSQL database dump
--

\restrict e80Sp0MZL8gTLkkh9Y06z6TRDfDnA6kezcllf9wMwCggbmfO8zntEhBKU9A5uAw

-- Dumped from database version 18.1
-- Dumped by pg_dump version 18.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: material_catalog; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.material_catalog (
    material_id integer NOT NULL,
    class_code character varying(50),
    group_code character varying(50),
    material_name character varying(255) NOT NULL
);


--
-- Name: material_catalog_material_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.material_catalog_material_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: material_catalog_material_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.material_catalog_material_id_seq OWNED BY public.material_catalog.material_id;


--
-- Name: storage_units; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.storage_units (
    order_number integer NOT NULL,
    date date NOT NULL,
    supplier_id integer,
    balance_sheet_account character varying(50),
    document_code character varying(50),
    document_number character varying(50),
    material_id integer,
    material_account character varying(50),
    unit_of_measure_code character varying(50) NOT NULL,
    quantity numeric(10,3) NOT NULL,
    unit_price numeric(10,2) NOT NULL,
    CONSTRAINT storage_units_quantity_check CHECK ((quantity > (0)::numeric)),
    CONSTRAINT storage_units_unit_price_check CHECK ((unit_price >= (0)::numeric))
);


--
-- Name: storage_units_order_number_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.storage_units_order_number_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: storage_units_order_number_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.storage_units_order_number_seq OWNED BY public.storage_units.order_number;


--
-- Name: suppliers; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.suppliers (
    supplier_id integer NOT NULL,
    name character varying(255) NOT NULL,
    tax_id character varying(12),
    legal_address_zip character varying(10),
    legal_address_city character varying(100),
    legal_address_street character varying(100),
    legal_address_house character varying(20),
    bank_address_zip character varying(10),
    bank_address_city character varying(100),
    bank_address_street character varying(100),
    bank_address_house character varying(20),
    bank_account_number character varying(50)
);


--
-- Name: suppliers_supplier_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.suppliers_supplier_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: suppliers_supplier_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.suppliers_supplier_id_seq OWNED BY public.suppliers.supplier_id;


--
-- Name: units_of_measure; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.units_of_measure (
    material_id integer NOT NULL,
    unit_name character varying(50) NOT NULL
);


--
-- Name: material_catalog material_id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.material_catalog ALTER COLUMN material_id SET DEFAULT nextval('public.material_catalog_material_id_seq'::regclass);


--
-- Name: storage_units order_number; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.storage_units ALTER COLUMN order_number SET DEFAULT nextval('public.storage_units_order_number_seq'::regclass);


--
-- Name: suppliers supplier_id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.suppliers ALTER COLUMN supplier_id SET DEFAULT nextval('public.suppliers_supplier_id_seq'::regclass);


--
-- Data for Name: material_catalog; Type: TABLE DATA; Schema: public; Owner: -
--

COPY public.material_catalog (material_id, class_code, group_code, material_name) FROM stdin;
5	LKM	KRASK	Краска акриловая белая
6	DEREV	BRUS	Брус сосновый 50x50x3000
7	METALL	ARM	Арматура строительная d12
8	ELEKTR	KAB	Кабель ВВГ-Пнг(А)-LS 3x2.5
\.


--
-- Data for Name: storage_units; Type: TABLE DATA; Schema: public; Owner: -
--

COPY public.storage_units (order_number, date, supplier_id, balance_sheet_account, document_code, document_number, material_id, material_account, unit_of_measure_code, quantity, unit_price) FROM stdin;
4	2025-11-01	1	\N	ПРИХ1	125	5	10.01	литры	500.000	550.50
5	2025-11-05	2	\N	ПРИХ2	0098	7	10.02	тонны	5.500	75000.00
6	2025-11-10	3	\N	ПРИХ3	10/11	8	10.01	метры	1200.000	95.20
\.


--
-- Data for Name: suppliers; Type: TABLE DATA; Schema: public; Owner: -
--

COPY public.suppliers (supplier_id, name, tax_id, legal_address_zip, legal_address_city, legal_address_street, legal_address_house, bank_address_zip, bank_address_city, bank_address_street, bank_address_house, bank_account_number) FROM stdin;
1	ООО "СтройМастер"	7701001001	\N	Москва	ул. Ленина, д. 10	\N	\N	\N	\N	\N	40702810100000001001
2	АО "МеталлСнаб"	5002002002	\N	Санкт-Петербург	Невский пр-т, д. 5	\N	\N	\N	\N	\N	40702810200000002002
3	ИП Иванов А.В.	2703003003	\N	Казань	ул. Пушкина, д. 1	\N	\N	\N	\N	\N	40702810300000003003
\.


--
-- Data for Name: units_of_measure; Type: TABLE DATA; Schema: public; Owner: -
--

COPY public.units_of_measure (material_id, unit_name) FROM stdin;
5	литры
6	штуки
6	метры
7	тонны
7	метры
8	метры
\.


--
-- Name: material_catalog_material_id_seq; Type: SEQUENCE SET; Schema: public; Owner: -
--

SELECT pg_catalog.setval('public.material_catalog_material_id_seq', 8, true);


--
-- Name: storage_units_order_number_seq; Type: SEQUENCE SET; Schema: public; Owner: -
--

SELECT pg_catalog.setval('public.storage_units_order_number_seq', 6, true);


--
-- Name: suppliers_supplier_id_seq; Type: SEQUENCE SET; Schema: public; Owner: -
--

SELECT pg_catalog.setval('public.suppliers_supplier_id_seq', 3, true);


--
-- Name: material_catalog material_catalog_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.material_catalog
    ADD CONSTRAINT material_catalog_pkey PRIMARY KEY (material_id);


--
-- Name: storage_units storage_units_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.storage_units
    ADD CONSTRAINT storage_units_pkey PRIMARY KEY (order_number);


--
-- Name: suppliers suppliers_bank_account_number_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.suppliers
    ADD CONSTRAINT suppliers_bank_account_number_key UNIQUE (bank_account_number);


--
-- Name: suppliers suppliers_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.suppliers
    ADD CONSTRAINT suppliers_pkey PRIMARY KEY (supplier_id);


--
-- Name: suppliers suppliers_tax_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.suppliers
    ADD CONSTRAINT suppliers_tax_id_key UNIQUE (tax_id);


--
-- Name: units_of_measure units_of_measure_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.units_of_measure
    ADD CONSTRAINT units_of_measure_pkey PRIMARY KEY (material_id, unit_name);


--
-- Name: storage_units storage_units_material_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.storage_units
    ADD CONSTRAINT storage_units_material_id_fkey FOREIGN KEY (material_id) REFERENCES public.material_catalog(material_id) ON DELETE RESTRICT;


--
-- Name: storage_units storage_units_material_id_unit_of_measure_code_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.storage_units
    ADD CONSTRAINT storage_units_material_id_unit_of_measure_code_fkey FOREIGN KEY (material_id, unit_of_measure_code) REFERENCES public.units_of_measure(material_id, unit_name) ON UPDATE CASCADE ON DELETE RESTRICT;


--
-- Name: storage_units storage_units_supplier_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.storage_units
    ADD CONSTRAINT storage_units_supplier_id_fkey FOREIGN KEY (supplier_id) REFERENCES public.suppliers(supplier_id) ON DELETE RESTRICT;


--
-- Name: units_of_measure units_of_measure_material_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.units_of_measure
    ADD CONSTRAINT units_of_measure_material_id_fkey FOREIGN KEY (material_id) REFERENCES public.material_catalog(material_id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

\unrestrict e80Sp0MZL8gTLkkh9Y06z6TRDfDnA6kezcllf9wMwCggbmfO8zntEhBKU9A5uAw

