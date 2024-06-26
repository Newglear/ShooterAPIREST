﻿PGDMP              
        |        	   shooterdb    16.2    16.2 +    C           0    0    ENCODING    ENCODING        SET client_encoding = 'UTF8';
                      false            D           0    0 
   STDSTRINGS 
   STDSTRINGS     (   SET standard_conforming_strings = 'on';
                      false            E           0    0 
   SEARCHPATH 
   SEARCHPATH     8   SELECT pg_catalog.set_config('search_path', '', false);
                      false            F           1262    16398 	   shooterdb    DATABASE     |   CREATE DATABASE shooterdb WITH TEMPLATE = template0 ENCODING = 'UTF8' LOCALE_PROVIDER = libc LOCALE = 'French_France.1252';
    DROP DATABASE shooterdb;
                shooteradmin    false            G           0    0 
   SCHEMA public    ACL     ,   GRANT ALL ON SCHEMA public TO shooteradmin;
                   pg_database_owner    false    6                        3079    16399    pgcrypto 	   EXTENSION     <   CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA public;
    DROP EXTENSION pgcrypto;
                   false            H           0    0    EXTENSION pgcrypto    COMMENT     <   COMMENT ON EXTENSION pgcrypto IS 'cryptographic functions';
                        false    2            �            1259    16473    achievement    TABLE     �   CREATE TABLE public.achievement (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name text NOT NULL,
    description text NOT NULL,
    image text
);
    DROP TABLE public.achievement;
       public         heap    shooteradmin    false            �            1259    16436    client    TABLE       CREATE TABLE public.client (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    username text NOT NULL,
    email text NOT NULL,
    password text NOT NULL,
    salt text NOT NULL,
    role_id integer DEFAULT 1 NOT NULL,
    rank_id integer DEFAULT 1 NOT NULL
);
    DROP TABLE public.client;
       public         heap    shooteradmin    false            �            1259    16483    client_achievement    TABLE     j   CREATE TABLE public.client_achievement (
    client_id uuid NOT NULL,
    achievement_id uuid NOT NULL
);
 &   DROP TABLE public.client_achievement;
       public         heap    shooteradmin    false            �            1259    16522    friend    TABLE     [   CREATE TABLE public.friend (
    client1_id uuid NOT NULL,
    client2_id uuid NOT NULL
);
    DROP TABLE public.friend;
       public         heap    shooteradmin    false            �            1259    16506    rank    TABLE     S   CREATE TABLE public.rank (
    id integer NOT NULL,
    rank_name text NOT NULL
);
    DROP TABLE public.rank;
       public         heap    shooteradmin    false            �            1259    16505    rank_id_seq    SEQUENCE     �   CREATE SEQUENCE public.rank_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 "   DROP SEQUENCE public.rank_id_seq;
       public          shooteradmin    false    222            I           0    0    rank_id_seq    SEQUENCE OWNED BY     ;   ALTER SEQUENCE public.rank_id_seq OWNED BY public.rank.id;
          public          shooteradmin    false    221            �            1259    16449    role    TABLE     i   CREATE TABLE public.role (
    role_id integer NOT NULL,
    role_name character varying(50) NOT NULL
);
    DROP TABLE public.role;
       public         heap    shooteradmin    false            �            1259    16448    role_role_id_seq    SEQUENCE     �   CREATE SEQUENCE public.role_role_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 '   DROP SEQUENCE public.role_role_id_seq;
       public          shooteradmin    false    218            J           0    0    role_role_id_seq    SEQUENCE OWNED BY     E   ALTER SEQUENCE public.role_role_id_seq OWNED BY public.role.role_id;
          public          shooteradmin    false    217            �           2604    16509    rank id    DEFAULT     b   ALTER TABLE ONLY public.rank ALTER COLUMN id SET DEFAULT nextval('public.rank_id_seq'::regclass);
 6   ALTER TABLE public.rank ALTER COLUMN id DROP DEFAULT;
       public          shooteradmin    false    221    222    222            �           2604    16452    role role_id    DEFAULT     l   ALTER TABLE ONLY public.role ALTER COLUMN role_id SET DEFAULT nextval('public.role_role_id_seq'::regclass);
 ;   ALTER TABLE public.role ALTER COLUMN role_id DROP DEFAULT;
       public          shooteradmin    false    218    217    218            <          0    16473    achievement 
   TABLE DATA           C   COPY public.achievement (id, name, description, image) FROM stdin;
    public          shooteradmin    false    219   �/       9          0    16436    client 
   TABLE DATA           W   COPY public.client (id, username, email, password, salt, role_id, rank_id) FROM stdin;
    public          shooteradmin    false    216   �/       =          0    16483    client_achievement 
   TABLE DATA           G   COPY public.client_achievement (client_id, achievement_id) FROM stdin;
    public          shooteradmin    false    220   �/       @          0    16522    friend 
   TABLE DATA           8   COPY public.friend (client1_id, client2_id) FROM stdin;
    public          shooteradmin    false    223   �/       ?          0    16506    rank 
   TABLE DATA           -   COPY public.rank (id, rank_name) FROM stdin;
    public          shooteradmin    false    222   0       ;          0    16449    role 
   TABLE DATA           2   COPY public.role (role_id, role_name) FROM stdin;
    public          shooteradmin    false    218   \0       K           0    0    rank_id_seq    SEQUENCE SET     9   SELECT pg_catalog.setval('public.rank_id_seq', 5, true);
          public          shooteradmin    false    221            L           0    0    role_role_id_seq    SEQUENCE SET     >   SELECT pg_catalog.setval('public.role_role_id_seq', 2, true);
          public          shooteradmin    false    217            �           2606    16480    achievement achievement_pkey 
   CONSTRAINT     Z   ALTER TABLE ONLY public.achievement
    ADD CONSTRAINT achievement_pkey PRIMARY KEY (id);
 F   ALTER TABLE ONLY public.achievement DROP CONSTRAINT achievement_pkey;
       public            shooteradmin    false    219            �           2606    16487 *   client_achievement client_achievement_pkey 
   CONSTRAINT        ALTER TABLE ONLY public.client_achievement
    ADD CONSTRAINT client_achievement_pkey PRIMARY KEY (client_id, achievement_id);
 T   ALTER TABLE ONLY public.client_achievement DROP CONSTRAINT client_achievement_pkey;
       public            shooteradmin    false    220    220            �           2606    16447    client client_email_key 
   CONSTRAINT     S   ALTER TABLE ONLY public.client
    ADD CONSTRAINT client_email_key UNIQUE (email);
 A   ALTER TABLE ONLY public.client DROP CONSTRAINT client_email_key;
       public            shooteradmin    false    216            �           2606    16443    client client_pkey 
   CONSTRAINT     P   ALTER TABLE ONLY public.client
    ADD CONSTRAINT client_pkey PRIMARY KEY (id);
 <   ALTER TABLE ONLY public.client DROP CONSTRAINT client_pkey;
       public            shooteradmin    false    216            �           2606    16445    client client_username_key 
   CONSTRAINT     Y   ALTER TABLE ONLY public.client
    ADD CONSTRAINT client_username_key UNIQUE (username);
 D   ALTER TABLE ONLY public.client DROP CONSTRAINT client_username_key;
       public            shooteradmin    false    216            �           2606    16526    friend friend_pkey 
   CONSTRAINT     d   ALTER TABLE ONLY public.friend
    ADD CONSTRAINT friend_pkey PRIMARY KEY (client1_id, client2_id);
 <   ALTER TABLE ONLY public.friend DROP CONSTRAINT friend_pkey;
       public            shooteradmin    false    223    223            �           2606    16482    achievement name_unique 
   CONSTRAINT     R   ALTER TABLE ONLY public.achievement
    ADD CONSTRAINT name_unique UNIQUE (name);
 A   ALTER TABLE ONLY public.achievement DROP CONSTRAINT name_unique;
       public            shooteradmin    false    219            �           2606    16513    rank rank_pkey 
   CONSTRAINT     L   ALTER TABLE ONLY public.rank
    ADD CONSTRAINT rank_pkey PRIMARY KEY (id);
 8   ALTER TABLE ONLY public.rank DROP CONSTRAINT rank_pkey;
       public            shooteradmin    false    222            �           2606    16515    rank rank_rank_name_key 
   CONSTRAINT     W   ALTER TABLE ONLY public.rank
    ADD CONSTRAINT rank_rank_name_key UNIQUE (rank_name);
 A   ALTER TABLE ONLY public.rank DROP CONSTRAINT rank_rank_name_key;
       public            shooteradmin    false    222            �           2606    16454    role role_pkey 
   CONSTRAINT     Q   ALTER TABLE ONLY public.role
    ADD CONSTRAINT role_pkey PRIMARY KEY (role_id);
 8   ALTER TABLE ONLY public.role DROP CONSTRAINT role_pkey;
       public            shooteradmin    false    218            �           2606    16493 9   client_achievement client_achievement_achievement_id_fkey 
   FK CONSTRAINT     �   ALTER TABLE ONLY public.client_achievement
    ADD CONSTRAINT client_achievement_achievement_id_fkey FOREIGN KEY (achievement_id) REFERENCES public.achievement(id);
 c   ALTER TABLE ONLY public.client_achievement DROP CONSTRAINT client_achievement_achievement_id_fkey;
       public          shooteradmin    false    220    219    4761            �           2606    16488 4   client_achievement client_achievement_client_id_fkey 
   FK CONSTRAINT     �   ALTER TABLE ONLY public.client_achievement
    ADD CONSTRAINT client_achievement_client_id_fkey FOREIGN KEY (client_id) REFERENCES public.client(id);
 ^   ALTER TABLE ONLY public.client_achievement DROP CONSTRAINT client_achievement_client_id_fkey;
       public          shooteradmin    false    4755    220    216            �           2606    16455    client client_role_id_fkey 
   FK CONSTRAINT     }   ALTER TABLE ONLY public.client
    ADD CONSTRAINT client_role_id_fkey FOREIGN KEY (role_id) REFERENCES public.role(role_id);
 D   ALTER TABLE ONLY public.client DROP CONSTRAINT client_role_id_fkey;
       public          shooteradmin    false    4759    216    218            �           2606    16516    client fk_rank_id 
   FK CONSTRAINT     o   ALTER TABLE ONLY public.client
    ADD CONSTRAINT fk_rank_id FOREIGN KEY (rank_id) REFERENCES public.rank(id);
 ;   ALTER TABLE ONLY public.client DROP CONSTRAINT fk_rank_id;
       public          shooteradmin    false    222    216    4767            �           2606    16527    friend friend_client1_id_fkey 
   FK CONSTRAINT     �   ALTER TABLE ONLY public.friend
    ADD CONSTRAINT friend_client1_id_fkey FOREIGN KEY (client1_id) REFERENCES public.client(id);
 G   ALTER TABLE ONLY public.friend DROP CONSTRAINT friend_client1_id_fkey;
       public          shooteradmin    false    4755    223    216            �           2606    16532    friend friend_client2_id_fkey 
   FK CONSTRAINT     �   ALTER TABLE ONLY public.friend
    ADD CONSTRAINT friend_client2_id_fkey FOREIGN KEY (client2_id) REFERENCES public.client(id);
 G   ALTER TABLE ONLY public.friend DROP CONSTRAINT friend_client2_id_fkey;
       public          shooteradmin    false    223    4755    216            <   
   x������ � �      9   
   x������ � �      =   
   x������ � �      @   
   x������ � �      ?   9   x�3�t*�ϫJ�2�t,JO�+�2��/�2��I,��+��2�t�L���K����� `?
�      ;      x�3��I�L-�2�tq����� 6�}     