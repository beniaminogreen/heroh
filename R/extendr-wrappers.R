# Generated by extendr: Do not edit by hand

# nolint start

#
# This file was created with the following call:
#   .Call("wrap__make_workwme_wrappers", use_symbols = TRUE, package_name = "workwme")

#' @docType package
#' @usage NULL
#' @useDynLib workwme, .registration = TRUE
NULL

send_to_other <- function(name, x) invisible(.Call(wrap__send_to_other, name, x))

receive <- function(ticket_id) .Call(wrap__receive, ticket_id)


# nolint end
