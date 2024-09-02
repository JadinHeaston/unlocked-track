# Unlocked Track

A Cybersecurity tool for reporting organization machines that were found to be unlocked! (Bad!!!)

This tool was designed to allow Cybersecurity to easily report these machines, either by having this application on a shared network drive or keeping it on a USB drive.

## Table of Contents <!-- omit in toc -->

1. [Unlocked Track](#unlocked-track)
	1. [Configuration](#configuration)
		1. [Constants](#constants)
	2. [Building The Application](#building-the-application)

## Configuration

Update [main.rs](src/main.rs) constants. (The section prefixed with `//CONSTANTS` that has variables with `CAPITALIZED_NAMES`)

### Constants

| Variable Name     | Description                                                                                             | Default Value                                 |
| ----------------- | ------------------------------------------------------------------------------------------------------- | --------------------------------------------- |
| PIN_LIST          | An array of PINs that are accepted by the application.                                                  | `[12345]`                                     |
| EMAIL_SMTP_HOST   | The SMTP hostname. By default, TLS is disabled and the port is `25`. The can be updated if you want. :) | `smtp.example.com`                            |
| EMAIL_SENDER      | The name and email the report should be sent from.                                                      | `unlocked-track <unlocked-track@example.com>` |
| EMAIL_DESTINATION | The name and email of the destination the report should be sent to.                                     | `Cybersecurity <cybersecurity@example.com>`   |
| EMAIL_SUBJECT     | The subject of the email sent to Cybersecurity.                                                         | `unlocked-track`                              |

## Building The Application

`cargo build --release`
