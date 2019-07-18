--
-- PostgreSQL database dump
--

-- Dumped from database version 11.4
-- Dumped by pg_dump version 11.4

-- Started on 2019-07-18 13:44:24

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

DROP DATABASE album;
--
-- TOC entry 2860 (class 1262 OID 16396)
-- Name: album; Type: DATABASE; Schema: -; Owner: bgael
--

CREATE DATABASE album WITH TEMPLATE = template0 ENCODING = 'UTF8' LC_COLLATE = 'fr_FR.UTF-8' LC_CTYPE = 'fr_FR.UTF-8';


ALTER DATABASE album OWNER TO bgael;

\connect album

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- TOC entry 2 (class 3079 OID 16416)
-- Name: pgcrypto; Type: EXTENSION; Schema: -; Owner: 
--

CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA public;


--
-- TOC entry 2861 (class 0 OID 0)
-- Dependencies: 2
-- Name: EXTENSION pgcrypto; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION pgcrypto IS 'cryptographic functions';


SET default_tablespace = '';

SET default_with_oids = false;

--
-- TOC entry 197 (class 1259 OID 16397)
-- Name: picture; Type: TABLE; Schema: public; Owner: bgael
--

CREATE TABLE public.picture (
    id integer NOT NULL,
    data text NOT NULL,
    model character varying
);


ALTER TABLE public.picture OWNER TO bgael;

--
-- TOC entry 198 (class 1259 OID 16403)
-- Name: picture_Id_seq; Type: SEQUENCE; Schema: public; Owner: bgael
--

CREATE SEQUENCE public."picture_Id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."picture_Id_seq" OWNER TO bgael;

--
-- TOC entry 2862 (class 0 OID 0)
-- Dependencies: 198
-- Name: picture_Id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bgael
--

ALTER SEQUENCE public."picture_Id_seq" OWNED BY public.picture.id;


--
-- TOC entry 199 (class 1259 OID 16408)
-- Name: user; Type: TABLE; Schema: public; Owner: bgael
--

CREATE TABLE public."user" (
    email character varying NOT NULL,
    password character varying NOT NULL,
    id integer NOT NULL
);


ALTER TABLE public."user" OWNER TO bgael;

--
-- TOC entry 200 (class 1259 OID 16453)
-- Name: user_id_seq; Type: SEQUENCE; Schema: public; Owner: bgael
--

CREATE SEQUENCE public.user_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.user_id_seq OWNER TO bgael;

--
-- TOC entry 2863 (class 0 OID 0)
-- Dependencies: 200
-- Name: user_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bgael
--

ALTER SEQUENCE public.user_id_seq OWNED BY public."user".id;


--
-- TOC entry 2730 (class 2604 OID 16405)
-- Name: picture id; Type: DEFAULT; Schema: public; Owner: bgael
--

ALTER TABLE ONLY public.picture ALTER COLUMN id SET DEFAULT nextval('public."picture_Id_seq"'::regclass);


--
-- TOC entry 2731 (class 2604 OID 16455)
-- Name: user id; Type: DEFAULT; Schema: public; Owner: bgael
--

ALTER TABLE ONLY public."user" ALTER COLUMN id SET DEFAULT nextval('public.user_id_seq'::regclass);


--
-- TOC entry 2733 (class 2606 OID 16407)
-- Name: picture pk_picture; Type: CONSTRAINT; Schema: public; Owner: bgael
--

ALTER TABLE ONLY public.picture
    ADD CONSTRAINT pk_picture PRIMARY KEY (id);


-- Completed on 2019-07-18 13:44:24

--
-- PostgreSQL database dump complete
--

