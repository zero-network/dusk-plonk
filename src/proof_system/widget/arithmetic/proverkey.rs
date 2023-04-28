// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use crate::fft::Evaluations;
use zero_crypto::common::Pairing;
use zero_kzg::Polynomial;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct ProverKey<P: Pairing> {
    pub q_m: (Polynomial<P::ScalarField>, Evaluations<P>),
    pub q_l: (Polynomial<P::ScalarField>, Evaluations<P>),
    pub q_r: (Polynomial<P::ScalarField>, Evaluations<P>),
    pub q_o: (Polynomial<P::ScalarField>, Evaluations<P>),
    pub q_c: (Polynomial<P::ScalarField>, Evaluations<P>),
    pub q_4: (Polynomial<P::ScalarField>, Evaluations<P>),
    pub q_arith: (Polynomial<P::ScalarField>, Evaluations<P>),
}

impl<P: Pairing> ProverKey<P> {
    pub(crate) fn compute_quotient_i(
        &self,
        index: usize,
        a_w_i: &P::ScalarField,
        b_w_i: &P::ScalarField,
        c_w_i: &P::ScalarField,
        d_w_i: &P::ScalarField,
    ) -> P::ScalarField {
        let q_m_i = &self.q_m.1[index];
        let q_l_i = &self.q_l.1[index];
        let q_r_i = &self.q_r.1[index];
        let q_o_i = &self.q_o.1[index];
        let q_c_i = &self.q_c.1[index];
        let q_4_i = &self.q_4.1[index];
        let q_arith_i = &self.q_arith.1[index];

        // (a(x)b(x)q_M(x) + a(x)q_L(x) + b(X)q_R(x) + c(X)q_O(X) + d(x)q_4(X) +
        // Q_C(X)) * Q_Arith(X)
        //
        let a_1 = *a_w_i * b_w_i * q_m_i;
        let a_2 = *a_w_i * q_l_i;
        let a_3 = *b_w_i * q_r_i;
        let a_4 = *c_w_i * q_o_i;
        let a_5 = *d_w_i * q_4_i;
        let a_6 = q_c_i;
        (a_1 + a_2 + a_3 + a_4 + a_5 + a_6) * q_arith_i
    }

    pub(crate) fn compute_linearization(
        &self,
        a_eval: &P::ScalarField,
        b_eval: &P::ScalarField,
        c_eval: &P::ScalarField,
        d_eval: &P::ScalarField,
        q_arith_eval: &P::ScalarField,
    ) -> Polynomial<P::ScalarField> {
        let q_m_poly = &self.q_m.0;
        let q_l_poly = &self.q_l.0;
        let q_r_poly = &self.q_r.0;
        let q_o_poly = &self.q_o.0;
        let q_c_poly = &self.q_c.0;
        let q_4_poly = &self.q_4.0;

        // (a_eval * b_eval * q_m_poly + a_eval * q_l + b_eval * q_r + c_eval
        // * q_o + d_eval * q_4 + q_c) * q_arith_eval
        //
        // a_eval * b_eval * q_m_poly
        let ab = *a_eval * b_eval;
        let a_0 = q_m_poly * &ab;

        // a_eval * q_l
        let a_1 = q_l_poly * a_eval;

        // b_eval * q_r
        let a_2 = q_r_poly * b_eval;

        //c_eval * q_o
        let a_3 = q_o_poly * c_eval;

        // d_eval * q_4
        let a_4 = q_4_poly * d_eval;

        let mut a = a_0 + a_1;
        a = a + a_2;
        a = a + a_3;
        a = a + a_4;
        a = &a + q_c_poly;
        a = &a * q_arith_eval;

        a
    }
}
