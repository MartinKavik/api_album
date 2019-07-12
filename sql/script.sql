--
-- PostgreSQL database dump
--

-- Dumped from database version 11.3
-- Dumped by pg_dump version 11.3

-- Started on 2019-07-12 20:49:51 CEST

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
-- TOC entry 3911 (class 1262 OID 16617)
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

SET default_tablespace = '';

SET default_with_oids = false;

--
-- TOC entry 196 (class 1259 OID 16618)
-- Name: picture; Type: TABLE; Schema: public; Owner: bgael
--

CREATE TABLE public.picture (
    id integer NOT NULL,
    data text NOT NULL
);


ALTER TABLE public.picture OWNER TO bgael;

--
-- TOC entry 197 (class 1259 OID 16621)
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
-- TOC entry 3912 (class 0 OID 0)
-- Dependencies: 197
-- Name: picture_Id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bgael
--

ALTER SEQUENCE public."picture_Id_seq" OWNED BY public.picture.id;


--
-- TOC entry 198 (class 1259 OID 24810)
-- Name: user; Type: TABLE; Schema: public; Owner: bgael
--

CREATE TABLE public."user" (
    id integer NOT NULL,
    email character varying NOT NULL,
    password character varying NOT NULL
);


ALTER TABLE public."user" OWNER TO bgael;

--
-- TOC entry 3780 (class 2604 OID 16623)
-- Name: picture id; Type: DEFAULT; Schema: public; Owner: bgael
--

ALTER TABLE ONLY public.picture ALTER COLUMN id SET DEFAULT nextval('public."picture_Id_seq"'::regclass);


--
-- TOC entry 3782 (class 2606 OID 16628)
-- Name: picture pk_picture; Type: CONSTRAINT; Schema: public; Owner: bgael
--

ALTER TABLE ONLY public.picture
    ADD CONSTRAINT pk_picture PRIMARY KEY (id);


--
-- TOC entry 3784 (class 2606 OID 24814)
-- Name: user pk_user; Type: CONSTRAINT; Schema: public; Owner: bgael
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT pk_user PRIMARY KEY (id);


-- Completed on 2019-07-12 20:49:51 CEST

--
-- PostgreSQL database dump complete
--

