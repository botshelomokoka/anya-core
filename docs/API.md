# Anya Core API Documentation

## Table of Contents
1. [Introduction](#introduction)
2. [Authentication](#authentication)
3. [Endpoints](#endpoints)
    - [User Management](#user-management)
    - [Bitcoin Operations](#bitcoin-operations)
    - [Lightning Network](#lightning-network)
    - [Stacks (STX) Support](#stacks-stx-support)
    - [Discreet Log Contracts (DLCs)](#discreet-log-contracts-dlcs)
    - [ML Fee Management](#ml-fee-management)
    - [DAO Governance](#dao-governance)
4. [Error Handling](#error-handling)
5. [Rate Limiting](#rate-limiting)
6. [Versioning](#versioning)

## Introduction
This document provides a comprehensive guide to the Anya Core API, detailing the available endpoints, request/response formats, and authentication methods.

## Authentication
All API requests require authentication using JSON Web Tokens (JWT). Include the JWT in the Authorization header of your requests:
