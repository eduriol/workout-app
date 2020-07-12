--
-- PostgreSQL database dump
--

-- Dumped from database version 12.3
-- Dumped by pg_dump version 12.3

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
-- Data for Name: workouts; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.workouts (id, muscular_group) FROM stdin;
1	upper
3	upper
4	lower
5	lower
\.


--
-- Data for Name: exercises; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.exercises (id, name, sets, reps, workout_id) FROM stdin;
4	pull-ups	3	8	1
5	incline dumbbell bench press	3	10	1
6	bent over row	4	5	1
7	overhead press	3	10	1
8	chin-ups	3	10	1
9	skullcrusher	3	10	1
2	barbell bench press	4	5	1
1	squats	4	5	4
3	deadlift	4	5	4
10	leg press	3	15	4
11	seated leg curl	3	10	4
12	standing calf raise	3	10	4
13	incline barbell bench press	3	12	3
14	flat bench dumbbell flye	3	12	3
15	seated cable row	3	12	3
16	one arm dumbbell row	3	12	3
17	dumbbell lateral raise	3	12	3
18	seated incline dumbbell curl	3	12	3
19	cable triceps extension	3	12	3
20	front squat	3	12	5
21	barbell lunge	3	12	5
22	leg extension	3	15	5
23	lying leg curl	3	15	5
24	seated calf raise	3	12	5
25	calf press	3	12	5
\.


--
-- Name: exercises_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.exercises_id_seq', 25, true);


--
-- Name: workouts_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.workouts_id_seq', 5, true);


--
-- PostgreSQL database dump complete
--

--
-- PostgreSQL database dump
--

-- Dumped from database version 12.3
-- Dumped by pg_dump version 12.3

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
-- Data for Name: workouts; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.workouts (id, muscular_group) FROM stdin;
1	upper
3	upper
4	lower
5	lower
\.


--
-- Data for Name: exercises; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.exercises (id, name, sets, max_reps, workout_id, min_reps) FROM stdin;
4	pull-ups	3	8	1	5
5	incline dumbbell bench press	3	10	1	6
6	bent over row	4	5	1	3
7	overhead press	3	10	1	6
9	skullcrusher	3	10	1	6
2	barbell bench press	4	5	1	3
1	squats	4	5	4	3
3	deadlift	4	5	4	3
10	leg press	3	15	4	10
11	seated leg curl	3	10	4	6
12	standing calf raise	3	10	4	6
13	incline barbell bench press	3	12	3	8
14	flat bench dumbbell flye	3	12	3	8
15	seated cable row	3	12	3	8
16	one arm dumbbell row	3	12	3	8
17	dumbbell lateral raise	3	12	3	8
18	seated incline dumbbell curl	3	12	3	8
19	cable triceps extension	3	12	3	8
20	front squat	3	12	5	8
21	barbell lunge	3	12	5	8
22	leg extension	3	15	5	10
23	lying leg curl	3	15	5	10
24	seated calf raise	3	12	5	8
25	calf press	3	12	5	8
8	chin-ups	3	8	1	5
\.


--
-- Name: exercises_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.exercises_id_seq', 25, true);


--
-- Name: workouts_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.workouts_id_seq', 5, true);


--
-- PostgreSQL database dump complete
--

